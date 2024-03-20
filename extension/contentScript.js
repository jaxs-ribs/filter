function replaceText(node) {
    if (node.nodeType === 3) {
        node.nodeValue = node.nodeValue.replace(/the/g, 'AAA');
    } else {
        node.childNodes.forEach(replaceText);
    }
}

function observeMutations(element) {
    const observer = new MutationObserver((mutations) => {
        mutations.forEach((mutation) => {
            mutation.addedNodes.forEach(replaceText);
        });
    });

    observer.observe(element, {
        childList: true,
        subtree: true
    });
}

replaceText(document.body);
observeMutations(document.body);