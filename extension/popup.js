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
    document.getElementById('port').addEventListener('change', function() {
        const portValue = this.value;
        chrome.storage.local.set({'port': portValue}, function() {
            console.log('Port value saved:', portValue);
        });
    });
});

function fetchSettings() {
    chrome.storage.local.get(['port'], function(result) {
        const port = result.port || '8080'; 
        document.getElementById('port').value = port; 

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
        textareaElement.addEventListener('input', debounceSubmitSettings); 
        textareaElement.addEventListener('input', autoResize, false); // Add this line
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
    textareaElement.addEventListener('input', debounceSubmitSettings); 
    textareaElement.addEventListener('input', autoResize, false); // Add this line
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

function debounceSubmitSettings() {
    clearTimeout(timeoutId);

    timeoutId = setTimeout(() => {
        submitSettings();
    }, 20); 
}

function submitSettings() {
    document.getElementById('port').addEventListener('change', function() {
        const port = this.value;
        chrome.storage.local.set({port: port}, function() {
            console.log('Port is set to ' + port);
        });
    });
    const port = document.getElementById('port').value || '8080';
    const rules = Array.from(document.querySelectorAll('.rule')).map(input => input.value);
    const is_on = document.getElementById('toggle').checked;

    fetch(`http://localhost:${port}/filter:filter:template.os/submit_settings`, {
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

function autoResize() {
    this.style.height = 'auto';
    this.style.height = (this.scrollHeight) + 'px';
}