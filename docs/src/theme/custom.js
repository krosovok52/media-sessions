// Custom JavaScript for Media Sessions Documentation

// Language switcher
document.addEventListener('DOMContentLoaded', function() {
    // Add language switcher
    const header = document.querySelector('.book-header');
    if (header) {
        const langSwitcher = document.createElement('div');
        langSwitcher.className = 'language-switcher';
        langSwitcher.innerHTML = `
            <a href="index.html" class="active">🇬🇧 EN</a>
            <a href="ru/index.html">🇷🇺 RU</a>
        `;
        header.appendChild(langSwitcher);
    }
    
    // Add theme toggle
    const themeToggle = document.createElement('button');
    themeToggle.className = 'theme-toggle';
    themeToggle.innerHTML = '🌓 Theme';
    themeToggle.onclick = function() {
        const body = document.body;
        if (body.classList.contains('light')) {
            body.classList.remove('light');
            body.classList.add('coal');
            localStorage.setItem('mdbook-theme', 'coal');
        } else {
            body.classList.remove('coal');
            body.classList.add('light');
            localStorage.setItem('mdbook-theme', 'light');
        }
    };
    document.body.appendChild(themeToggle);
    
    // Set current language active
    const currentPath = window.location.pathname;
    if (currentPath.includes('/ru/')) {
        const links = document.querySelectorAll('.language-switcher a');
        links.forEach(link => {
            if (link.textContent.includes('RU')) {
                link.classList.add('active');
            } else {
                link.classList.remove('active');
            }
        });
    }
    
    // Add copy buttons to code blocks
    const codeBlocks = document.querySelectorAll('pre code');
    codeBlocks.forEach(codeBlock => {
        const pre = codeBlock.parentElement;
        const copyBtn = document.createElement('button');
        copyBtn.className = 'copy-button';
        copyBtn.innerHTML = '📋 Copy';
        copyBtn.onclick = function() {
            navigator.clipboard.writeText(codeBlock.textContent);
            copyBtn.innerHTML = '✅ Copied!';
            setTimeout(() => {
                copyBtn.innerHTML = '📋 Copy';
            }, 2000);
        };
        pre.appendChild(copyBtn);
    });
});
