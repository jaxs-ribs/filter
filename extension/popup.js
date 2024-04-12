document.addEventListener('DOMContentLoaded', function() {
    fetchSettings();
    document.getElementById('add-rule').addEventListener('click', function() {
        addRule();
    });
    document.getElementById('remove-rule').addEventListener('click', function() {
        removeRule();
    });
    document.getElementById('port').addEventListener('change', function() {
        const portValue = this.value;
        chrome.storage.local.set({'port': portValue}, function() {
            console.log('Port value saved:', portValue);
        });
    });
    document.getElementById('api-key').addEventListener('change', function() {
        const apiKeyValue = this.value;
        chrome.storage.local.set({'api_key': apiKeyValue}, function() {
            console.log('API Key value saved:', apiKeyValue);
        });
    });
    document.getElementById('save').addEventListener('click', submitSettings);
});

function fetchSettings() {
    chrome.storage.local.get(['port', 'api_key'], function(result) {
        const port = result.port || '8080'; 
        document.getElementById('port').value = port; 

        const apiKey = result.api_key || '';
        document.getElementById('api-key').value = apiKey;

        fetch(`http://localhost:${port}/filter:filter:template.os/fetch_settings`, {
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
    });
}

function displayRules(rules) {
    const container = document.getElementById('rules-container');
    container.innerHTML = ''; 
    rules.forEach(rule => {
        const textareaElement = document.createElement('textarea'); 
        textareaElement.classList.add('rule');
        textareaElement.value = rule; 
        textareaElement.addEventListener('input', autoResize, false); 
        container.appendChild(textareaElement);
    });
}

function setToggleState(is_on) {
    const toggle = document.getElementById('toggle');
    toggle.checked = is_on;
}

function addRule() {
    const container = document.getElementById('rules-container');
    const textareaElement = document.createElement('textarea');
    textareaElement.classList.add('rule');
    textareaElement.placeholder = "Enter a rule";
    textareaElement.addEventListener('input', autoResize, false); 
    container.appendChild(textareaElement);
}

function removeRule() {
    const container = document.getElementById('rules-container');
    const lastRuleElement = container.lastElementChild;
    if (lastRuleElement) {
        container.removeChild(lastRuleElement);
    }
}

let timeoutId;

function submitSettings() {
    const port = document.getElementById('port').value || '8080';
    const rules = Array.from(document.querySelectorAll('.rule')).map(input => input.value);
    const is_on = document.getElementById('toggle').checked;
    const api_key = document.getElementById('api-key').value; 

    fetch(`http://localhost:${port}/filter:filter:template.os/submit_settings`, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify({
            rules: rules,
            is_on: is_on,
            api_key: api_key 
        })
    }).then(response => {
        if (!response.ok) {
            throw new Error('Network response was not ok');
        }
    }).catch(error => console.error('Error submitting settings:', error));

    fetchSettings();
}

function autoResize() {
    this.style.height = 'auto';
    this.style.height = (this.scrollHeight) + 'px';
}