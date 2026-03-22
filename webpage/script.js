const canvas = document.getElementById('hero-canvas');
const ctx = canvas.getContext('2d');
const container = canvas.parentElement;

let drawing = false;
let points = [];

// Set canvas size to match container
function resize() {
    canvas.width = container.clientWidth;
    canvas.height = container.clientHeight;
    
    // Clear background
    ctx.fillStyle = '#000';
    ctx.fillRect(0, 0, canvas.width, canvas.height);
}

window.addEventListener('resize', resize);
resize();

// Simple drawing logic
function startDrawing(e) {
    drawing = true;
    points = [];
    addPoint(e);
}

function stopDrawing() {
    drawing = false;
}

function addPoint(e) {
    const rect = canvas.getBoundingClientRect();
    const x = (e.clientX || e.touches[0].clientX) - rect.left;
    const y = (e.clientY || e.touches[0].clientY) - rect.top;
    
    points.push({ x, y });
    draw();
}

function draw() {
    if (!drawing || points.length < 2) return;
    
    ctx.strokeStyle = '#00d1ff';
    ctx.lineWidth = 3;
    ctx.lineJoin = 'round';
    ctx.lineCap = 'round';
    
    ctx.beginPath();
    ctx.moveTo(points[0].x, points[0].y);
    
    for (let i = 1; i < points.length; i++) {
        ctx.lineTo(points[i].x, points[i].y);
    }
    
    ctx.stroke();
}

canvas.addEventListener('mousedown', startDrawing);
canvas.addEventListener('mousemove', (e) => {
    if (drawing) addPoint(e);
});
window.addEventListener('mouseup', stopDrawing);

// Touch support
canvas.addEventListener('touchstart', (e) => {
    e.preventDefault();
    startDrawing(e);
});
canvas.addEventListener('touchmove', (e) => {
    e.preventDefault();
    if (drawing) addPoint(e);
});
canvas.addEventListener('touchend', stopDrawing);

// Subtle initial animation: draw "Sway"
function autoDraw() {
    const text = "Sway-Draw";
    ctx.font = 'bold 40px Inter';
    ctx.fillStyle = 'rgba(0, 209, 255, 0.1)';
    ctx.textAlign = 'center';
    ctx.textBaseline = 'middle';
    ctx.fillText(text, canvas.width / 2, canvas.height / 2);
    
    // Some random lines to look like annotations
    ctx.strokeStyle = 'rgba(0, 209, 255, 0.3)';
    ctx.setLineDash([5, 15]);
    ctx.beginPath();
    ctx.moveTo(50, 50);
    ctx.bezierCurveTo(200, 100, 100, 300, 350, 350);
    ctx.stroke();
}

// Copy to clipboard functionality
const copyBtn = document.getElementById('copy-btn');
const installCode = document.getElementById('install-code');

if (copyBtn && installCode) {
    copyBtn.addEventListener('click', () => {
        const text = installCode.innerText;
        navigator.clipboard.writeText(text).then(() => {
            copyBtn.classList.add('success');
            const originalIcon = copyBtn.innerHTML;
            copyBtn.innerHTML = '<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="20 6 9 17 4 12"></polyline></svg>';
            
            setTimeout(() => {
                copyBtn.classList.remove('success');
                copyBtn.innerHTML = originalIcon;
            }, 2000);
        });
    });
}

setTimeout(autoDraw, 500);

