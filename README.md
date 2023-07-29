# openAI_cli_chatbog_codetothemoon_tutorial

I. What this app does:

This is a simple CLI app that prompts the user to ask ChatGPT a question and receive a response.

II. How to set up your OpenAI API key as an environmental variable

It's good practice to not put out logins and passwords like an API key out into the wild (and onto Github). In this simple example, I stored my OpenAI API key in the .env file that I added to this director when I opened it up in my code editor. You're welcome to hardcode your API key onto the main.rs in your code editor, but do not publiclly reveal it! After you use up $5 worth of data, OpenAI will begin to charge you $$$. It's best practice to an .env file and add your API key to the OPENAI_TOKEN variable in that .env file, which will then be cited on line 49.

III. Why are all my responses about Jindos?

As you can see in line 47 of main.rs, my Preamble is part of the prompt which shapes the responses I expect to get from ChatGPT. Since my rescue dog is a Jindo, I wanted the responses we got back to reflect a major player in my life: my dog!

IV. To run the app, you can simply clone this repo and run 'Cargo Run' in your terminal.
