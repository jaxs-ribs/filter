// Add at the top
function fetchStatusAndReplaceText() {
    fetch('http://localhost:8080/filter:filter:template.os/status', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({})
    })
    .then(response => {
        if (!response.ok) {
            throw new Error('Network response was not ok');
        }
        return response.text();
    })
    .then(status => {
        console.log("Fetched status:", status); // Added line to print the response text in console
        replaceText(document.body, status);
        observeMutations(document.body, status);
    })
    .catch(error => console.error("Failed to fetch status:", error));
}

// Modify the replaceText function to accept status
function replaceText(node, status) {
    if (node.nodeType === 3) {
        node.nodeValue = node.nodeValue.replace(/the/g, status);
    } else {
        node.childNodes.forEach(child => replaceText(child, status));
    }
}

// Modify the observeMutations function to accept status
function observeMutations(element, status) {
    const observer = new MutationObserver((mutations) => {
        mutations.forEach((mutation) => {
            mutation.addedNodes.forEach(node => replaceText(node, status));
        });
    });

    observer.observe(element, {
        childList: true,
        subtree: true
    });
}

// Replace direct calls with the new function
fetchStatusAndReplaceText();