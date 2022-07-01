console.log(JSON.parse(localStorage.getItem("user_data")))
let user_token = JSON.parse(JSON.parse(localStorage.getItem("user_data")))
console.log(user_token.user_id)
 
 ajax(`https://4930-188-130-155-167.ngrok.io/api/user/${user_token.user_id}/quiz`, "GET", response_reg)

// console.log(data.quiz_number)
// console.log(($("#username")).text())
// console.log(document.querySelectorAll("#quiz_button")[0].getElementsByClassName("title")[0].innerHTML)

function response_reg(data){
    console.log(data);
    addQuiz(JSON.parse(data).quiz_number-1);
    $("#username").text(JSON.parse(data).user.name);
    console.log(JSON.parse(data).quiz_number)
    for (let i = 0; i < JSON.parse(data).quiz_number;i+=1){
      
        document.querySelectorAll("#quiz_button")[i].getElementsByClassName("title")[0].innerHTML = JSON.parse(data).quizzes[i].name

    }
        
    
}
document.querySelector("#add_quiz").onclick = function(event){
    event.preventDefault();
    window.location.href="../templates/page_of_create.html"
    

}
// response_reg(data)