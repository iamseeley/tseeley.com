
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
  
  
  document.addEventListener('DOMContentLoaded', () => {
    setActiveLink();

  });