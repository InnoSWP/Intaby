let questions_array=[]

document.querySelector("#btn_create").onclick = function(event){
    

let answer_array=[]
let  question_type;
let time_mode;
    event.preventDefault();

    let quiz_name = $("#quiz_name");  
    let question = $("#question");
    question_type = $('#answerType').val();
    time_mode = $("#select_time");
    // console.log( $('#answerType').val())
    // console.log( $('#select_time').val())
    var answerButtons = document.getElementsByClassName('answer_wrapper');
    console.log(answerButtons)



   
    
    let i = 0;
    $(".Answer_type").each(function(){
        // console.log(this)
        console.log(answerButtons[i].classList.contains("selected"))
       
        // console.log(i)
        // let cor_answ = answerButtons[i].classList.contains("selected")
        answer_json={
            "text":this.value,
            "correct_answer": answerButtons[i].classList.contains("selected"),
        }
        i+=1;
     
        //push answers to array
        answer_array.push(answer_json)
        this.value =""
    })
    if (question_type==1){
        question_type = "Quiz"
    }
    else if (question_type == 2){
        question_type = "Multiquiz"
    }
    else if (question_type == 3){
        question_type = "Poll"
    }
    question_json = {
        "question_type":question_type,
        "text":question.val(),
        "time":parseInt(time_mode.val()),
        "answers": answer_array
    } 
    questions_array.push(question_json)
    
    question.val("");
    // time_mode.querySelector('[selected]').selected = true

document.querySelector("#btn_save").onclick = function(event){
    
    
  
    //data to request
    data = {

        "name":quiz_name.val(),
        "questions":questions_array,
    }
    let user_id_data = (JSON.parse(JSON.parse(localStorage.getItem("user_data")))).user_id
    // console.log(user_id_data)
    let request_url = `http://51.250.19.225:8888/api/user/${user_id_data}/quiz`
    console.log(data)
    // console.log(request_url)
     ajax(request_url, "POST", response_reg, JSON.stringify(data));
    function response_reg(dataArr){
      
             window.location.href = "/page_of_user.html"
       
      
    }
    // console.log(questions_array)
    console.log(data)

}



}
