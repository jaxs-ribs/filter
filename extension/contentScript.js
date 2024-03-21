let globalTweetMap = new Map();

async function retrieveAndModifyTweetContents() {
    const tweets = document.querySelectorAll("article");
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

        if (content && !globalTweetMap.has(content)) {
            // Send the tweet for modification immediately
            const requestBody = JSON.stringify({ tweet: content });
            console.log("Sending tweet for modification:", requestBody);
            try {
                const response = await fetch('http://localhost:8080/filter:filter:template.os/send', {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    body: requestBody
                });
                const data = await response.json();
                // Adjusting to correctly access the modified tweet from the server's response
                const modifiedText = data.tweet || "Modification failed"; // Use the correct key here
                globalTweetMap.set(content, modifiedText);

                // Now, modify the tweet content on the webpage
                textsDom.forEach(textDom => {
                    if (textDom.tagName.toLowerCase() === "span") {
                        textDom.innerText = modifiedText;
                    }
                    // For images, consider how you want to handle modifications
                });
            } catch (error) {
                console.error("Failed to modify tweet:", error);
            }
        }
    }
}

setInterval(retrieveAndModifyTweetContents, 5000); // Check for new tweets every 5 seconds