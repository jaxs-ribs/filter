document.addEventListener('DOMContentLoaded', function() {
    fetchSettings();
    document.getElementById('add-rule').addEventListener('click', addRule);
});

function fetchSettings() {
    console.log('fetchSettings');
    fetch('http://localhost:8080/filter:filter:template.os/fetch_settings', {
        method: 'POST', 
        headers: {
            'Content-Type': 'application/json' 
        },
        body: JSON.stringify({}) 
    })
    .then(response => response.json())
    .then(data => {
        displayRules(data.rules);
        setToggleState(data.is_on);
    })
    .catch(error => console.error('Error fetching settings:', error));
}

function displayRules(rules) {
    const container = document.getElementById('rules-container');
    container.innerHTML = ''; 
    rules.forEach(rule => {
        const inputElement = document.createElement('input'); // Changed from 'div' to 'input'
        inputElement.type = 'text'; // Specify that it's a text input
        inputElement.classList.add('rule');
        inputElement.value = rule; // Use value for input elements
        container.appendChild(inputElement);
    });
}

function setToggleState(is_on) {
    const toggle = document.getElementById('toggle');
    toggle.checked = is_on;
}

function addRule() {
    const container = document.getElementById('rules-container');
    const inputElement = document.createElement('input');
    inputElement.type = 'text';
    inputElement.classList.add('rule');
    inputElement.placeholder = "";
    container.appendChild(inputElement);
}
