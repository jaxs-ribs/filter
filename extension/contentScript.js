let globalTweetMap = new Map();

// TODO: Zena: When scrolling back up, the tweet is reloaded, but not in the hashmap anymore. Maybe not important for now though, maybe another time. 
async function retrieveAndModifyTweetContents() {
    const tweets = document.querySelectorAll("article");
    let newTweets = []; // Array to hold new tweets

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

        // Add new tweets to the array instead of sending them immediately
        if (content && !globalTweetMap.has(content)) {
            newTweets.push(content);
        }
    }

    // Check if there are new tweets to send
    if (newTweets.length > 0) {
        console.log("Sending new tweets for modification:", newTweets);
        try {
            const requestBody = JSON.stringify({ tweets: newTweets });
            const response = await fetch('http://localhost:8080/filter:filter:template.os/send', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: requestBody
            });
            // Adjust the part where you handle the server response
            const data = await response.json();
            // Access the 'tweets' key to get the array of modified tweets
            const modifiedTweets = data.tweets || []; // Ensure it falls back to an empty array if undefined

            // Iterate over the modifiedTweets array instead of the raw data object
            modifiedTweets.forEach((modifiedText, index) => {
                const originalText = newTweets[index];
                globalTweetMap.set(originalText, modifiedText || "Modification failed");

                // Find the original tweet DOM based on the content and modify it
                tweets.forEach(tweet => {
                    const textsDom = tweet.querySelectorAll("[data-testid=tweetText] > *");
                    let tweetContent = "";
                    textsDom.forEach(textDom => {
                        if (textDom.tagName.toLowerCase() === "span") {
                            tweetContent += textDom.innerText;
                        }
                        if (textDom.tagName.toLowerCase() === "img") {
                            tweetContent += textDom.getAttribute("alt");
                        }
                    });

                    if (tweetContent === originalText) {
                        // Now, modify the tweet content on the webpage
                        textsDom.forEach(textDom => {
                            if (textDom.tagName.toLowerCase() === "span") {
                                textDom.innerText = modifiedText || "Modification failed";
                            }
                            // For images, consider how you want to handle modifications
                        });
                    }
                });
            });
        } catch (error) {
            console.error("Failed to modify tweets:", error);
        }
    }
}

setInterval(retrieveAndModifyTweetContents, 5000); // Check for new tweets every 5 seconds