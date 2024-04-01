let globalTweetMap = new Map();

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

        // Check the number of bookmark buttons and duplicate if there is exactly one
        const bookmarkButtons = tweet.querySelectorAll('[data-testid="bookmark"]');
        if (bookmarkButtons.length === 1) {
            const bookmarkButton = bookmarkButtons[0];
            const cloneBookmarkButton = bookmarkButton.cloneNode(true);
            bookmarkButton.parentNode.insertBefore(cloneBookmarkButton, bookmarkButton);
        }

        if (content && !globalTweetMap.has(content)) {
            newTweets.push(content);
            // Initially mark unprocessed tweets as grey
            textsDom.forEach(textDom => {
                if (textDom.tagName.toLowerCase() === "span") {
                    textDom.style.color = "grey";
                }
            });
        }
    }

    if (newTweets.length > 0) {
        try {
            const requestBody = JSON.stringify({ tweets: newTweets });
            const response = await fetch('http://localhost:8080/filter:filter:template.os/send', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: requestBody,
            });
            const data = await response.json();
            const tweetResults = data.tweet_results || [];

            tweetResults.forEach((result, index) => {
                const originalText = newTweets[index];
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
                        if (result === false) {
                            // Hide the tweet text
                            textsDom.forEach(textDom => {
                                textDom.style.display = "none"; // Hide text
                            });
                    
                            // Check if the show button already exists to avoid adding it multiple times
                            /*
                            if (!tweet.querySelector("button")) { // This line checks for an existing button
                                const showButton = document.createElement("button");
                                showButton.innerText = "Show";
                                showButton.onclick = function() {
                                    textsDom.forEach(textDom => {
                                        textDom.style.display = ""; // Show text
                                    });
                                    showButton.remove(); // Remove the button after clicking
                                };
                                tweet.appendChild(showButton); // Add the show button to the tweet
                            }
                            */
                        } else {
                            // Modify the tweet content color based on the modification result
                            textsDom.forEach(textDom => {
                                if (textDom.tagName.toLowerCase() === "span") {
                                    textDom.style.color = "white";
                                }
                                // For images, consider how you want to handle modifications
                            });
                        }
                    }
                });
            });
        } catch (error) {
            console.error("Failed to modify tweets:", error);
        }
    }
}
setInterval(retrieveAndModifyTweetContents, 1000); // Check for new tweets every second