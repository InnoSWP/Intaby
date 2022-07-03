console.log(JSON.parse(localStorage.getItem("user_data")))
let user_token = JSON.parse(JSON.parse(localStorage.getItem("user_data")))
console.log(user_token)

ajax(`http://51.250.99.184:8888/api/user/${user_token.user_id}/quiz`, "GET", response_reg, user_token)

// console.log(data.quiz_number)
// console.log(($("#username")).text())
// console.log(document.querySelectorAll("#quiz_button")[0].getElementsByClassName("title")[0].innerHTML)

function response_reg(data){
    console.log(data);
    let dataArr= JSON.parse(data);
    
    $("#username").text(dataArr.user.name);
    localStorage.setItem("name", dataArr.user.name)
    
    addQuiz(dataArr.quiz_number-1);
    for (let i = 0; i < dataArr.quiz_number;i+=1){
        
        document.querySelectorAll("#quiz_button")[i].getElementsByClassName("title")[0].innerHTML = dataArr.quizzes[i].name;
       ($(".edit"))[i].id =  dataArr.quizzes[i].quiz_id;
       
    }
        
    
}


document.querySelector("#add_quiz").onclick = function(event){
    event.preventDefault();
    window.location.href="/page_of_create.html"
    

}


butttons = document.getElementsByClassName("edit");

let timerId = setTimeout(() => user(), 1000);

function user(){
  for (let i = 0; i < butttons.length; i+=1){
    butttons[i].onclick = (function(){
      localStorage.setItem("quiz_id", butttons[i].id);
      dataArr =  user_token.user_id
      console.log(butttons[i])
      console.log(butttons[i].id)
      ajax(`http://51.250.99.184:8080/games/${$(this).attr('id')}`, "POST", answ, dataArr)
      function answ(data){
        localStorage.setItem("code", data)
        window.location.href="/wh_teacher.html";
    
      }
    })
  }
  
}
