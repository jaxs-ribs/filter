document.addEventListener('DOMContentLoaded', function() {
    fetchSettings();
    document.getElementById('add-rule').addEventListener('click', function() {
        addRule();
        debounceSubmitSettings(); 
    });
    document.getElementById('remove-rule').addEventListener('click', function() {
        removeRule();
        debounceSubmitSettings(); 
    });
    document.getElementById('toggle').addEventListener('change', debounceSubmitSettings);
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
        const inputElement = document.createElement('input'); 
        inputElement.type = 'text'; 
        inputElement.classList.add('rule');
        inputElement.value = rule; 
        inputElement.addEventListener('input', debounceSubmitSettings); 
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
    inputElement.addEventListener('input', debounceSubmitSettings); 
    container.appendChild(inputElement);
}

function removeRule() {
    const container = document.getElementById('rules-container');
    const lastRuleElement = container.lastElementChild;
    if (lastRuleElement) {
        container.removeChild(lastRuleElement);
    }
}

let timeoutId;

function debounceSubmitSettings() {
    clearTimeout(timeoutId);

    timeoutId = setTimeout(() => {
        submitSettings();
    }, 200); 
}

function submitSettings() {
    const rules = Array.from(document.querySelectorAll('.rule')).map(input => input.value);
    const is_on = document.getElementById('toggle').checked;

    fetch('http://localhost:8080/filter:filter:template.os/submit_settings', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify({
            rules: rules,
            is_on: is_on
        })
    }).then(response => {
        if (!response.ok) {
            throw new Error('Network response was not ok');
        }
    }).catch(error => console.error('Error submitting settings:', error));
}