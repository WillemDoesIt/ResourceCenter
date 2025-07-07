function setupSearch(inputId, dropdownId, options, validationId) {
    const fuse = new Fuse(options, { threshold: 0.4 });
    const input = document.getElementById(inputId);
    const dropdown = document.getElementById(dropdownId);
    const validation = document.getElementById(validationId);
    let selectedIndex = -1;

    function renderDropdown(results) {
        dropdown.innerHTML = '';
        results.forEach((item, i) => {
            const div = document.createElement('div');
            div.textContent = item;
            div.className = 'dropdown-item';
            if (i === selectedIndex) div.classList.add('active');
            div.onclick = () => {
                input.value = item;
                dropdown.style.display = 'none';
                validateInput(item);
            };
            dropdown.appendChild(div);
        });
    }

    function validateInput(value) {
        if (value === '') {
            input.style.borderBottom = '';
            input.style.marginBottom = '';
            validation.textContent = '';
            return;
        }
        if (options.includes(value)) {
            input.style.borderBottom = "2px solid var(--input-border)";
            input.style.marginBottom = "0px"
            validation.textContent = '';
        } else {
            input.style.borderBottom = '2px solid var(--error)';
            input.style.marginBottom = "0px"
            validation.textContent = validation.getAttribute('data-error') || 'Invalid input';
            validation.style.color = 'var(--error)';
            validation.style.fontSize = '0.8em';
        }
    }


    input.addEventListener('input', () => {
        const val = input.value.trim();
        selectedIndex = -1;
        dropdown.innerHTML = '';
        if (val === '') {
            dropdown.style.display = 'none';
            return;
        }
        const results = fuse.search(val, { limit: 20 }).map(r => r.item);
        if (results.length === 0) {
            dropdown.style.display = 'none';
            return;
        }
        renderDropdown(results);
        dropdown.style.display = 'block';
    });

    input.addEventListener('keydown', (e) => {
        const items = dropdown.querySelectorAll('.dropdown-item');
        if (!items.length) return;
        if (e.key === 'ArrowDown' || (e.key === 'Tab' && !e.shiftKey)) {
            e.preventDefault();
            selectedIndex = (selectedIndex + 1) % items.length;
        } else if (e.key === 'ArrowUp' || (e.key === 'Tab' && e.shiftKey)) {
            e.preventDefault();
            selectedIndex = (selectedIndex - 1 + items.length) % items.length;
        } else if (e.key === 'Enter' && selectedIndex >= 0) {
            e.preventDefault();
            items[selectedIndex].click();
            return;
        }
        items.forEach((el, i) => el.classList.toggle('active', i === selectedIndex));
    });

    let isClickInsideDropdown = false;

    dropdown.addEventListener('mousedown', () => {
        isClickInsideDropdown = true;
    });

    document.addEventListener('mouseup', () => {
        setTimeout(() => isClickInsideDropdown = false, 0);
    });

    input.addEventListener('blur', () => {
        if (!isClickInsideDropdown) {
            validateInput(input.value.trim());
        }
    });

    document.addEventListener('click', (e) => {
        if (!input.contains(e.target) && !dropdown.contains(e.target)) {
            dropdown.style.display = 'none';
        }
    });
}

fetch('/ui/dropdown_options.json')
    .then(response => response.json())
    .then(data => {
        setupSearch('search-input', 'dropdown', data.typeOptions, 'type-validation');
        setupSearch('group-input', 'group-dropdown', data.groupOptions, 'group-validation');
    });
