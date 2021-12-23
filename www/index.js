import * as sim from 'lib-simulation-wasm';

CanvasRenderingContext2D.prototype.drawTriangle = function (
	x,
	y,
	size,
	rotation
) {
	this.beginPath();
	this.moveTo(x + Math.cos(rotation) * 1.5, y + Math.sin(rotation) * 1.5);

	this.lineTo(
		x + Math.cos(rotation + (2.0 / 3.0) * Math.PI) * size,
		y + Math.sin(rotation + (2.0 / 3.0) * Math.PI) * size
	);
	this.lineTo(
		x + Math.cos(rotation - (2.0 / 3.0) * Math.PI) * size,
		y + Math.sin(rotation - (2.0 / 3.0) * Math.PI) * size
	);
	this.lineTo(x, y);

	this.lineTo(x + Math.cos(rotation) * 1.5, y + Math.sin(rotation) * 1.5);
	this.stroke();
};

const simulation = new sim.Simulation();
const viewport = document.getElementById('viewport');
const viewportWidth = viewport.width;
const viewportHeight = viewport.height;

const viewportScale = window.devicePixelRatio || 1;
viewport.width = viewportWidth * viewportScale;
viewport.height = viewportHeight * viewportScale;

viewport.style.width = viewportWidth + 'px';
viewport.style.height = viewportHeight + 'px';

const ctxt = viewport.getContext('2d');

ctxt.scale(viewportScale, viewportScale);

ctxt.fillStyle = 'rgb(0, 0, 0)';

for (const animal of simulation.world().animals) {
	ctxt.drawTriangle(
		animal.x * viewportWidth,
		animal.y * viewportHeight,
		0.025 * viewportWidth,
		animal.rotation
	);
}
