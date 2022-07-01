let user_token = JSON.parse(localStorage.getItem("user_data"));
$("#start_quiz").click(function(event){
    event.preventDefault()
    dataArr={
        "user_id":user_token,
        "state": "InProgress"
    }
    ajax("", "PUT", response_reg, dataArr) 
})