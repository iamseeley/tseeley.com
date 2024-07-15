document.addEventListener('DOMContentLoaded', () => {
  setActiveLink();
  createFlowerField();
  applyGlow()
});

const setActiveLink = () => {
  const links = document.querySelectorAll('nav a');
  const currentPath = window.location.pathname;

  links.forEach(link => {
    const href = link.getAttribute('href');
    if (href === currentPath) {
      link.classList.add('active');
    } else {
      link.classList.remove('active');
    }
  });
};

const flowerField = document.querySelector('.pixelated-flower-field');

const colors = {
  grass: ['#8b9e3e', '#6d8b3c', '#507739', '#346637', '#1b5a35'], // shades of green for grass
  flower: ['#c63d59', '#a3273f', '#820c26', '#e19c24', '#c87711', '#a35d00'], // shades for flowers
};

// Function to create a pixel
const createPixel = (color, x, y) => {
  const pixel = document.createElement('div');
  pixel.classList.add('pixel');
  pixel.style.backgroundColor = color;
  pixel.style.left = `${x}px`;
  pixel.style.top = `${y}px`;
  return pixel;
};

// Function to create a flower
const createFlower = (x, y) => {
  const flowerPattern = [
    "  X  ",
    " XXX ",
    "XXXXX",
    " XXX ",
    "  X  "
  ];
  for (let i = 0; i < flowerPattern.length; i++) {
    for (let j = 0; j < flowerPattern[i].length; j++) {
      if (flowerPattern[i][j] === 'X') {
        const color = colors.flower[Math.floor(Math.random() * colors.flower.length)];
        flowerField.appendChild(createPixel(color, x + j, y + i));
      }
    }
  }
};

// Create flowers within the flower field container
const createFlowerField = () => {
  const fieldWidth = flowerField.offsetWidth;
  for (let i = 0; i < fieldWidth; i += 10) {
    const x = i;
    const y = Math.floor(Math.random() * (40 - 5));
    createFlower(x, y);
  }
};

const applyGlow = () => {
  const links = document.querySelectorAll('a:not(nav a)');

  links.forEach(link => {
    const glowPoint = document.createElement('div');
    glowPoint.classList.add('glow-point');
    link.appendChild(glowPoint);

    link.addEventListener('mouseenter', () => {
      glowPoint.style.opacity = '1';
      glowPoint.style.transform = 'translate(-50%, -50%) scale(1)';
      glowPoint.style.filter = 'blur(15px)';
    });

    link.addEventListener('mouseleave', () => {
      glowPoint.style.opacity = '0';
      glowPoint.style.transform = 'translate(-50%, -50%) scale(0.1)';
      glowPoint.style.filter = 'blur(20px)';
    });
  });
};