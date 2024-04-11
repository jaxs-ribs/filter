let globalTweetMap = new Map();
let globalTweetImageMap = new Map();
let globalTweetFilterMap = new Map();

const debug = true;

function extractTweetId(tweet) {
    const linkElement = tweet.querySelector('a[href*="/status/"]');
    let tweetId = null;
    if (linkElement) {
        const href = linkElement.getAttribute('href');
        const statusPosition = href.indexOf('/status/') + 8; // +8 to move past '/status/'
        tweetId = href.substring(statusPosition);
    }
    return tweetId;
}

function extractTweetPhotoUrl(tweet) {
    const photoDiv = tweet.querySelector('[data-testid="tweetPhoto"]');
    let photoUrl = null;
    if (photoDiv) {
        const imgTag = photoDiv.querySelector('img');
        if (imgTag) {
            photoUrl = imgTag.getAttribute('src');
        }
    }
    return photoUrl;
}

function getContent(textsDom) {
    let content = "";
    for (const textDom of textsDom) {
        if (textDom.tagName.toLowerCase() === "span") {
            content += textDom.innerText;
        }
        if (textDom.tagName.toLowerCase() === "img") {
            content += textDom.getAttribute("alt");
        }
    }
    return content;
}

function toggleVisibility(tweet, event) {
    event.stopPropagation(); // Prevent the event from bubbling up to parent elements
    const tweetId = extractTweetId(tweet);
    if (globalTweetFilterMap.has(tweetId)) {
        const currentValue = globalTweetFilterMap.get(tweetId);
        globalTweetFilterMap.set(tweetId, !currentValue);
    }
}

function insertLearnButton(tweet) {
    // Check the number of bookmark buttons and duplicate if there is exactly one
    const bookmarkButtons = tweet.querySelectorAll('[data-testid="bookmark"], [data-testid="removeBookmark"]');
    const clonedButtons = tweet.querySelectorAll('[data-testid="learnbutton"]');
    if (clonedButtons.length === 0 && bookmarkButtons.length > 0) {
        const bookmarkButton = bookmarkButtons[0];
        const cloneBookmarkButton = bookmarkButton.cloneNode(true);
        cloneBookmarkButton.removeAttribute('data-testid');
        cloneBookmarkButton.setAttribute('data-testid', 'learnbutton');
        cloneBookmarkButton.style.padding = "0 10px"; // Adjust padding as needed
        cloneBookmarkButton.addEventListener('click', function (event) {
            toggleVisibility(tweet, event);
        });
        const svgElement = cloneBookmarkButton.querySelector('svg');
        if (svgElement) {
            const newPath = "M18.3 5.71a1 1 0 00-1.41 0L12 10.59 7.11 5.7A1 1 0 005.7 7.11L10.59 12 5.7 16.89a1 1 0 101.41 1.41L12 13.41l4.89 4.89a1 1 0 001.41-1.41L13.41 12l4.89-4.89a1 1 0 000-1.4z";
            svgElement.querySelector('path').setAttribute('d', newPath);
        }
        bookmarkButton.parentNode.insertBefore(cloneBookmarkButton, bookmarkButton);
    }
}

function greyOutTweet(textsDom) {
    textsDom.forEach(textDom => {
        if (textDom.tagName.toLowerCase() === "span") {
            textDom.style.color = "grey";
        }
    });
}

async function updateVisuals() {
    const tweets = document.querySelectorAll("[data-testid=cellInnerDiv] > *");
    for (const tweet of tweets) {
        const textsDom = tweet.querySelectorAll("[data-testid=tweetText] > *");
        let tweetId = extractTweetId(tweet);
        insertLearnButton(tweet);
        if (!globalTweetMap.has(tweetId)) {
            greyOutTweet(textsDom);
        } else {
            let should_pass = globalTweetFilterMap.get(tweetId);
            if (should_pass) {
                if (tweet.firstChild) {
                    tweet.firstChild.style.display = ""; 
                    tweet.style.height = ""; 
                    tweet.style.cursor = "default"; 
                    tweet.onclick = null; 
                }
                textsDom.forEach(textDom => {
                    if (textDom.tagName.toLowerCase() === "span") {
                        textDom.style.color = "white";
                    }
                });
            } else {
                if (tweet.firstChild) {
                    tweet.firstChild.style.display = "none"; 
                    tweet.style.height = "5vh"; 
                    tweet.style.cursor = "pointer"; // Change cursor to pointer
                    tweet.onclick = function(event) {
                        toggleVisibility(tweet, event);
                    }
                }
            }
        }
    }
}

async function filterTweets() {
    const tweetsData = Array.from(globalTweetMap)
        .filter(([tweetId]) => !globalTweetFilterMap.has(tweetId))
        .map(([tweetId, content]) => {
            const photoUrl = globalTweetImageMap.has(tweetId) ? globalTweetImageMap.get(tweetId) : null;
            return { tweetId, content, photoUrl };
        });
    try {
        const requestBody = JSON.stringify({ tweets: tweetsData, debug: debug });
        console.log(requestBody);
        const response = await fetch('http://localhost:8080/filter:filter:template.os/filter', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: requestBody,
        });
        const data = await response.json();
        const filteredTweetResults = data || [];

        filteredTweetResults.forEach(({ tweetId, filterBool }) => {
            globalTweetFilterMap.set(tweetId, filterBool);
        });
    } catch (error) {
        console.error("Failed to filter tweets:", error);
    }
}

function populateGlobalTweetMap() {
    const tweets = document.querySelectorAll("article");
    for (const tweet of tweets) {
        const textsDom = tweet.querySelectorAll("[data-testid=tweetText] > *");

        let tweetId = extractTweetId(tweet);
        let content = getContent(textsDom);
        let photo = extractTweetPhotoUrl(tweet);

        if (tweetId && content) {
            globalTweetMap.set(tweetId, content);
            if (photo) {
                globalTweetImageMap.set(tweetId, photo);
            }
        }
    }
}

async function parseState() {
    populateGlobalTweetMap();
    await filterTweets();
}

parseState();
setInterval(updateVisuals, 100);
setInterval(parseState, 500);

