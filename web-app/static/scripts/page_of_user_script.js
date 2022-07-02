console.log(JSON.parse(localStorage.getItem("user_data")))
let user_token = JSON.parse(JSON.parse(localStorage.getItem("user_data")))
console.log(user_token)

ajax(`http://localhost:8888/api/user/${user_token.user_id}/quiz`, "GET", response_reg, user_token)

// console.log(data.quiz_number)
// console.log(($("#username")).text())
// console.log(document.querySelectorAll("#quiz_button")[0].getElementsByClassName("title")[0].innerHTML)

function response_reg(data){
    console.log(data);
    let dataArr= JSON.parse(data);
    
    $("#username").text(dataArr.user.name);
    addQuiz(dataArr.quiz_number-1);
    for (let i = 0; i < dataArr.quiz_number;i+=1){
        
        document.querySelectorAll("#quiz_button")[i].getElementsByClassName("title")[0].innerHTML = dataArr.quizzes[i].name;
       ($(".edit")).id =  dataArr.quiz_id
    }
        
    
}

console.log(($(".edit")))
document.querySelector("#add_quiz").onclick = function(event){
    event.preventDefault();
    window.location.href="/page_of_create.html"
    

}
$(".edit").click(function(event){
  event.preventDefault();
  
  localStorage.setItem("quiz_id", $(this).attr('id'));
  console.log($(this).attr('id'));
  window.location.href="/wh_teacher.html";
})
