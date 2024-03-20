// // Add at the top
// function fetchStatusAndReplaceText() {
//     fetch('http://localhost:8080/filter:filter:template.os/status', {
//         method: 'POST',
//         headers: { 'Content-Type': 'application/json' },
//         body: JSON.stringify({})
//     })
//     .then(response => {
//         if (!response.ok) {
//             throw new Error('Network response was not ok');
//         }
//         return response.text();
//     })
//     .then(status => {
//         console.log("Fetched status:", status); // Added line to print the response text in console
//         replaceText(document.body, status);
//         observeMutations(document.body, status);
//     })
//     .catch(error => console.error("Failed to fetch status:", error));
// }

// // Modify the replaceText function to accept status
// function replaceText(node, status) {
//     if (node.nodeType === 3) {
//         node.nodeValue = node.nodeValue.replace(/the/g, status);
//     } else {
//         node.childNodes.forEach(child => replaceText(child, status));
//     }
// }

// // Modify the observeMutations function to accept status
// function observeMutations(element, status) {
//     const observer = new MutationObserver((mutations) => {
//         mutations.forEach((mutation) => {
//             mutation.addedNodes.forEach(node => replaceText(node, status));
//         });
//     });

//     observer.observe(element, {
//         childList: true,
//         subtree: true
//     });
// }
// Replace direct calls with the new function
// fetchStatusAndReplaceText();

window.waitTill = (ms) => new Promise((r) => setTimeout(r, ms));

async function retrieveTweetContents() {
    let trial = 1;
    let tweets = document.querySelectorAll("article");

    while (trial < 5 && tweets.length === 0) {
        await window.waitTill(1000); // Wait for 1 second before retrying
        tweets = document.querySelectorAll("article");
        trial++;
    }

    let tweetContents = [];

    for (const tweet of tweets) {
        const textsDom = tweet.querySelectorAll("[data-testid=tweetText] > *");
        let content = "";

        for (const textDom of textsDom) {
            if (textDom.tagName.toLowerCase() === "span") {
                content += textDom.innerText;
            }
            if (textDom.tagName.toLowerCase() === "img") {
                content += textDom.getAttribute("alt");
            }
        }

        if (content) {
            tweetContents.push(content);
        }
    }

    // After fetching the tweets, send them to the server
    if (tweetContents.length > 0) {
        const requestBody = JSON.stringify({ tweets: tweetContents });
        console.log("Sending request body:", requestBody); // Log the body exactly as it's sent
        fetch('http://localhost:8080/filter:filter:template.os/send', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: requestBody
        })
            .then(response => {
                if (!response.ok) {
                    throw new Error('Network response was not ok');
                }
                return response.json(); // Assuming the server responds with JSON
            })
            .then(data => {
                console.log("Server response:", data);
            })
            .catch(error => {
                console.error("Failed to send tweets:", error);
            });
    }

    return tweetContents;
}

retrieveTweetContents().then((contents) => {
    console.log(contents);
});
fetch('http://localhost:8080/filter:filter:template.os/status', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
})
.then(response => {
    if (!response.ok) {
        throw new Error('Network response was not ok');
    }
    return response.json(); // Assuming the server responds with JSON
})
.then(data => {
    console.log("Status response:", data);
})
.catch(error => {
    console.error("Failed to get status:", error);
});

