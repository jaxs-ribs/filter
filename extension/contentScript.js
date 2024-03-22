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

        if (content && !globalTweetMap.has(content)) {
            newTweets.push(content);
        }
    }

    if (newTweets.length > 0) {
        console.log("Sending new tweets for modification:", newTweets);
        try {
            const requestBody = JSON.stringify({ tweets: newTweets });
            const response = await fetch('http://localhost:8080/filter:filter:template.os/send', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: requestBody,
            });
            const data = await response.json();
            // Assuming 'tweet_results' is an array of booleans corresponding to each tweet
            const tweetResults = data.tweet_results || [];

            tweetResults.forEach((result, index) => {
                const originalText = newTweets[index];
                // Store the result (true or false) directly in the globalTweetMap
                globalTweetMap.set(originalText, result);
                
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
                        // Modify the tweet content color based on the modification result
                        textsDom.forEach(textDom => {
                            if (textDom.tagName.toLowerCase() === "span") {
                                textDom.style.color = result ? "green" : "grey";
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