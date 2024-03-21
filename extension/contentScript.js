let globalTweetContents = new Set();

async function retrieveTweetContents() {
    async function fetchTweets() {
        let tweets = document.querySelectorAll("article");
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
                globalTweetContents.add(content); // Update the global set
            }
        }
    }

    await fetchTweets(); // Initial fetch
    setInterval(async () => {
        await fetchTweets(); // Fetch tweets every 5 seconds
        console.log("Updated tweets set:", Array.from(globalTweetContents));
    }, 5000);
}

retrieveTweetContents();
