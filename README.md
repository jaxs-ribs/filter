# Filter

Filters out the noise on Twitter. Add any rules for filtering tweets on the frontend, and an LLM will filter them out of your tweets in (near) real-time!

## How to Install

Ensure your node is running. You can either download "filter" from the app store or run the following command:

```bash
kit bs
```

Go to `chrome://extensions/`, click `Load Unpacked`, and then select the `pkg/extension` folder.

Finally, click the Chrome extension icon, fill in the port your kinode is running on, your OpenAI key, and your desired rules. Press save, and you're done!
