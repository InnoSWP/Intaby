let user_token = JSON.parse(localStorage.getItem("user_data"));
let code = localStorage.getItem("code");
let quiz_id_ = localStorage.getItem("quiz_id")
dataArr = user_token.user_id

$("#start_quiz").click(function(event){
    ajax(`/games/${quiz_id_}`, "POST", answ, dataArr) 
    function answ(data){
        event.preventDefault()
        dataArr={
            "user_id":user_token.user_id,
            "state": "InProgress"
        }
        ajax(`/games/${code}/state`, "PUT", response_reg, dataArr) 
        function response_reg(data){
            window.location.href = "/page_of_answer.html";

        }
    }
   
})  