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
    let tweetElements = []; // Store references to tweet DOM elements

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
            tweetElements.push(tweet); // Add the tweet element to the array
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
                // Assuming data.tweets is the array of modified tweet texts
                data.tweets.forEach((modifiedText, index) => {
                    // Find the text container within the tweet element
                    const textContainer = tweetElements[index].querySelector("[data-testid=tweetText]");
                    if (textContainer) {
                        // Replace the text content
                        textContainer.innerText = modifiedText;
                    }
                });
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
    body: 'this is a test'
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

