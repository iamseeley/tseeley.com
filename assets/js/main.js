document.addEventListener('DOMContentLoaded', () => {
  setActiveLink();
  // const allFlowers = createFlowerField();
  // applyGlow();
  // setupFloatingEffect(allFlowers);
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
  grass: ['#8b9e3e', '#6d8b3c', '#507739', '#346637', '#1b5a35'],
  flower: ['#c63d59', '#a3273f', '#820c26', '#e19c24', '#c87711', '#a35d00'],
};

const createPixel = (color, x, y) => {
  const pixel = document.createElement('div');
  pixel.classList.add('pixel');
  pixel.style.backgroundColor = color;
  pixel.style.left = `${x}px`;
  pixel.style.top = `${y}px`;
  return pixel;
};

const createFlower = (x, y) => {
  const flowerPattern = [
    " X ",
    " XXX ",
    "XXXXX",
    " XXX ",
    " X "
  ];
  const flowerPixels = [];
  const flowerContainer = document.createElement('div');
  flowerContainer.classList.add('flower-container');
  flowerContainer.style.position = 'absolute';
  flowerContainer.style.left = `${x}px`;
  flowerContainer.style.top = `${y}px`;

  for (let i = 0; i < flowerPattern.length; i++) {
    for (let j = 0; j < flowerPattern[i].length; j++) {
      if (flowerPattern[i][j] === 'X') {
        const color = colors.flower[Math.floor(Math.random() * colors.flower.length)];
        const pixel = createPixel(color, j, i);
        flowerPixels.push(pixel);
        flowerContainer.appendChild(pixel);
      }
    }
  }
  flowerField.appendChild(flowerContainer);
  return flowerContainer;
};

const createFlowerField = () => {
  const fieldWidth = flowerField.offsetWidth;
  const allFlowers = [];
  for (let i = 0; i < fieldWidth; i += 10) {
    const x = i;
    const y = Math.floor(Math.random() * (40 - 5));
    allFlowers.push(createFlower(x, y));
  }
  return allFlowers;
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

const animateFlowers = (flowers) => {
  let animationFrameId;
  const speeds = flowers.map(() => ({ x: 0, y: 0 }));
  
  const animate = () => {
    flowers.forEach((flower, index) => {
      const rect = flower.getBoundingClientRect();
      const fieldRect = flowerField.getBoundingClientRect();
      
      speeds[index].x += (Math.random() - 0.5) * 0.1;
      speeds[index].y += (Math.random() - 0.5) * 0.1;
      
      speeds[index].x *= 0.95;
      speeds[index].y *= 0.95;
      
      let newX = rect.left + speeds[index].x - fieldRect.left;
      let newY = rect.top + speeds[index].y - fieldRect.top;
      
      newX = Math.max(0, Math.min(newX, fieldRect.width - rect.width));
      newY = Math.max(0, Math.min(newY, fieldRect.height - rect.height));
      
      flower.style.left = `${newX}px`;
      flower.style.top = `${newY}px`;
    });
    animationFrameId = requestAnimationFrame(animate);
  };
  
  animate();
  return () => cancelAnimationFrame(animationFrameId);
};

const setupFloatingEffect = (allFlowers) => {
  let stopAnimation;
  let isAnimating = false;
  let animationTimeout;

  const startAnimation = () => {
    if (!isAnimating) {
      isAnimating = true;
      stopAnimation = animateFlowers(allFlowers);
    }
  };

  const stopAnimationWithDelay = (delay = 0) => {
    clearTimeout(animationTimeout);
    animationTimeout = setTimeout(() => {
      if (isAnimating && stopAnimation) {
        stopAnimation();
        isAnimating = false;
      }
    }, delay);
  };

  // Mouse events
  flowerField.addEventListener('mouseenter', startAnimation);
  flowerField.addEventListener('mouseleave', () => stopAnimationWithDelay());

  // Touch events
  flowerField.addEventListener('touchstart', (e) => {
    e.preventDefault(); // Prevent mouse events from firing
    startAnimation();
    stopAnimationWithDelay(3000); // Stop after 3 seconds
  });

  // Stop animation when the user scrolls or moves away
  document.addEventListener('scroll', () => stopAnimationWithDelay());
  document.addEventListener('touchmove', () => stopAnimationWithDelay());
};



