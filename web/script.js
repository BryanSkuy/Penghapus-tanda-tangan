// Elements
const dropZone = document.getElementById('drop-zone');
const fileInput = document.getElementById('file-input');
const previewArea = document.getElementById('preview-area');
const originalImg = document.getElementById('original-img');
const resultCanvas = document.getElementById('result-canvas');
const hiddenCanvas = document.getElementById('hidden-canvas');
const resetBtn = document.getElementById('reset-btn');
const downloadBtn = document.getElementById('download-btn');

// Sliders and Values
const sliders = {
    threshold: { el: document.getElementById('threshold'), val: document.getElementById('threshold-val') },
    tolerance: { el: document.getElementById('tolerance'), val: document.getElementById('tolerance-val') },
    contrast: { el: document.getElementById('contrast'), val: document.getElementById('contrast-val') }
};

let currentImageFile = null;
let originalImageData = null;

// --- Event Listeners ---

// Update values and trigger processing when sliders change
Object.keys(sliders).forEach(key => {
    sliders[key].el.addEventListener('input', (e) => {
        sliders[key].val.textContent = e.target.value;
        if (originalImageData) processImage();
    });
});

// Drag & Drop
['dragenter', 'dragover', 'dragleave', 'drop'].forEach(eventName => {
    dropZone.addEventListener(eventName, preventDefaults, false);
});
function preventDefaults(e) {
    e.preventDefault(); e.stopPropagation();
}

['dragenter', 'dragover'].forEach(eventName => {
    dropZone.addEventListener(eventName, () => dropZone.classList.add('dragover'), false);
});
['dragleave', 'drop'].forEach(eventName => {
    dropZone.addEventListener(eventName, () => dropZone.classList.remove('dragover'), false);
});

dropZone.addEventListener('drop', (e) => {
    let dt = e.dataTransfer;
    let files = dt.files;
    handleFiles(files);
});

fileInput.addEventListener('change', function() {
    handleFiles(this.files);
});

resetBtn.addEventListener('click', () => {
    dropZone.style.display = 'flex';
    previewArea.style.display = 'none';
    downloadBtn.disabled = true;
    originalImageData = null;
    currentImageFile = null;
    fileInput.value = '';
});

downloadBtn.addEventListener('click', () => {
    if (!originalImageData) return;
    const dataUrl = resultCanvas.toDataURL('image/png');
    const a = document.createElement('a');
    a.href = dataUrl;
    a.download = 'signature_transparent.png';
    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
});

// --- Core Logic ---

function handleFiles(files) {
    if (files.length === 0) return;
    const file = files[0];
    
    if (!file.type.startsWith('image/')) {
        alert("Pilih file gambar yang valid!");
        return;
    }

    currentImageFile = file;
    const reader = new FileReader();
    
    reader.onload = (e) => {
        const img = new Image();
        img.onload = () => {
            // Display in UI
            dropZone.style.display = 'none';
            previewArea.style.display = 'flex';
            downloadBtn.disabled = false;
            
            originalImg.src = e.target.result;
            
            // Extract image data using hidden canvas
            hiddenCanvas.width = img.width;
            hiddenCanvas.height = img.height;
            const ctx = hiddenCanvas.getContext('2d');
            ctx.drawImage(img, 0, 0);
            originalImageData = ctx.getImageData(0, 0, img.width, img.height);
            
            // Setup result canvas dimensions
            resultCanvas.width = img.width;
            resultCanvas.height = img.height;
            
            // First process
            processImage();
        };
        img.src = e.target.result;
    };
    reader.readAsDataURL(file);
}

// Emulate Rust logic in Javascript
function processImage() {
    if (!originalImageData) return;

    const threshold = parseInt(sliders.threshold.el.value, 10);
    const tolerance = parseInt(sliders.tolerance.el.value, 10);
    const contrastRaw = parseFloat(sliders.contrast.el.value); // -100 to 100

    // Contrast Factor Calculation (same as Rust)
    let contrastFactor = 1.0;
    if (contrastRaw !== 0) {
        contrastFactor = (259.0 * (contrastRaw + 255.0)) / (255.0 * (259.0 - contrastRaw));
    }

    const width = originalImageData.width;
    const height = originalImageData.height;
    
    // Create new image data for output
    const ctx = resultCanvas.getContext('2d');
    const resultData = ctx.createImageData(width, height);
    
    const src = originalImageData.data;
    const dst = resultData.data;

    for (let i = 0; i < src.length; i += 4) {
        let r = src[i];
        let g = src[i+1];
        let b = src[i+2];
        let a = src[i+3];

        // 1. Appy Contrast
        if (contrastRaw !== 0) {
            r = truncate(contrastFactor * (r - 128) + 128);
            g = truncate(contrastFactor * (g - 128) + 128);
            b = truncate(contrastFactor * (b - 128) + 128);
        }

        // 2. Base on Original data logic
        // Write the calculated RGB to dst (Rust writes original or modified depending on implementation, 
        // we'll write the high contrast version for better ink)
        dst[i] = r;
        dst[i+1] = g;
        dst[i+2] = b;

        // 3. Luminance calculation
        const luma = (0.299 * r) + (0.587 * g) + (0.114 * b);

        // 4. White neutrality check
        const maxVal = Math.max(r, g, b);
        const minVal = Math.min(r, g, b);
        const diff = maxVal - minVal;
        const isNeutral = diff <= tolerance;

        // 5. Alpha logic
        if (luma > threshold && isNeutral) {
            dst[i+3] = 0; // Transparent
        } else {
            dst[i+3] = a; // Keep original alpha (usually 255)
        }
    }

    // Advanced: Feathering (Edge smoothing)
    // To keep it fast in JS, we implement a simplified feathering:
    // If a transparent pixel touches an opaque pixel, give it semi-transparency.
    const dstCopy = new Uint8ClampedArray(dst);
    for (let y = 1; y < height - 1; y++) {
        for (let x = 1; x < width - 1; x++) {
            const idx = (y * width + x) * 4;
            const alpha = dstCopy[idx + 3];

            if (alpha === 0) {
                // Check neighbors
                let darkNeighbors = 0;
                let sumAlpha = 0;

                const offsets = [
                    -width - 1, -width, -width + 1,
                    -1,                  1,
                     width - 1,  width,  width + 1
                ];

                for (let j = 0; j < 8; j++) {
                    const nIdx = idx + (offsets[j] * 4);
                    const nAlpha = dstCopy[nIdx + 3];
                    if (nAlpha > 128) {
                        darkNeighbors++;
                        sumAlpha += nAlpha;
                    }
                }

                if (darkNeighbors > 0) {
                    // It's an edge pixel on the transparent side
                    dst[idx + 3] = sumAlpha / 8; // Feathering effect
                }
            } else if (alpha > 0 && alpha < 255) {
                // Smooth existing semi-transparent pixels further if needed
            }
        }
    }

    ctx.putImageData(resultData, 0, 0);
}

function truncate(val) {
    if (val < 0) return 0;
    if (val > 255) return 255;
    return val;
}
