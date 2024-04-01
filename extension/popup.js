document.addEventListener('DOMContentLoaded', function() {
    fetchSettings();
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
        const ruleElement = document.createElement('div');
        ruleElement.classList.add('rule');
        ruleElement.textContent = rule;
        container.appendChild(ruleElement);
    });
}

function setToggleState(is_on) {
    const toggle = document.getElementById('toggle');
    toggle.checked = is_on;
}
