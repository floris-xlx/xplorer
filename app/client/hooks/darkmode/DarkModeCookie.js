// Function to set the theme in the document and save it to localStorage
export function setTheme(theme) {
  if (typeof document !== "undefined") {
    // Set the data-theme attribute on the document's root element
    document.documentElement.setAttribute('data-theme', theme);
    // Save the theme to localStorage
    localStorage.setItem('theme', theme);
  }
}


// Function to get the current theme from localStorage
export function getTheme() {
  if (typeof document !== "undefined") {
    // Retrieve the theme from localStorage
    let theme = localStorage.getItem('theme');
    if (!theme) {
      // If no theme is found, default to 'light' and set it
      theme = 'light';
      setTheme(theme);
    } else {
      // If a theme is found, set it
      setTheme(theme);
    }
    return theme;
  }
  // Return null if document is undefined (e.g., in a server-side environment)
  return null;
}
