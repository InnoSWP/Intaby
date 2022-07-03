let user_token = JSON.parse(JSON.parse(localStorage.getItem("user_data")));
let code = localStorage.getItem("code");
let quiz_id_ = localStorage.getItem("quiz_id")
console.log(user_token.user_id)
dataArr = {
    "user_id":user_token.user_id
}
dataArr2 = {
    "user_id":user_token.user_id,
    "state": "InProgress"
}

 let timerId = setInterval(() => wait_hall_t(), 500);
wait_hall_t()
function wait_hall_t(){
    ajax(`https://bfd5-188-130-155-167.ngrok.io/games/${code}`, "GET", response_reg, dataArr)
    function response_reg(dataArr3){
        
        console.log(JSON.parse(dataArr3).players.length);

        console.log($("#people_in_lobby"));
        ($("#people_in_lobby"))[0].innerHTML = "People in lobby: " + (JSON.parse(dataArr3).players.length);
        document.getElementsByClassName("management_quiz-code-number")[0].innerHTML = code;
    }

    $("#start_quiz").click(function(event){
        ajax(`https://bfd5-188-130-155-167.ngrok.io/games/${code}/state`, "PUT", response_reg, JSON.stringify(dataArr2));
           function response_reg(data){
               
                // window.location.href = "../templates/page_of_answer.html";
           }

      
      
   })  
}
