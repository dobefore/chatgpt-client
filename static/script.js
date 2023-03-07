document.addEventListener("DOMContentLoaded", () => {
const chatbox = document.getElementById("chatbox");
const message = document.getElementById("message");
const form = document.getElementById("form");

form.addEventListener("submit", async (e) => {
    e.preventDefault();
    const userMessage = message.value;
    message.value = "";
    const botMessage =await getBotMessage(userMessage);
    var converter = new showdown.Converter();
        html      = converter.makeHtml(botMessage);
    addMessage(userMessage, "user");
    setTimeout(() => {
        addMessage(html, "bot");
    }, 1000);
});

function addMessage(message, sender) {
    const className = sender === "user" ? "user-message" : "bot-message";
    const div = document.createElement("div");
    div.classList.add(className);
    div.innerHTML = message;
    chatbox.append(div);
}

async function getBotMessage(userMessage) {

     if (userMessage !== '') {
        try {
          const response = await fetch('/ask', {
            method: 'POST',
            headers: {
              'Content-Type': 'application/json'
            },
            body: JSON.stringify({question: userMessage })
          });
          const json = await response.json();
          const text=JSON.parse(json).answer;
return text
        }catch (error) {
          alert(error);
          console.error(error);
        }
    }
    // const model = await tf.loadGraphModel("https://cdn.openai.com/gpt-3/models/117M/"); // Load GPT-3 117M model
    // const response = await model.execute({
    //     input: userMessage,
    //     use_cache: true,
    //     temperature: 0.5,
    //     max_length: 50
    // });
    // const botMessage = response[0].join("");
    // model.dispose();
    // return botMessage;
}

})

