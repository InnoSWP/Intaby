
let questions_array=[]
let answer_array=[]
let  question_type;
let time_mode;

document.querySelector("#btn_create").onclick = function(event){
    event.preventDefault();

    let quiz_name = $("#quiz_name");  
    let question = $("#question");
    question_type = $('#select_type').val();
    time_mode = $("#select_time");
    console.log( $('#select_type').val())
    console.log( $('#select_time').val())
    var answerButtons = document.getElementsByClassName('answer_wrapper')
    console.log(answerButtons[0].lastChild)

    console.log(document.getElementsByClassName("Badge"))


   
    
 
    $(".Answer_type").each(function(){
        // console.log(this)

        answer_json={
            "text":this.value,
            "correct_answer": 1,
        }
     
        //push answers to array
        answer_array.push(answer_json)
        this.value =""
    })
    question_json = {
        "question_type":question_type,
        "text":question.val(),
        "time":time_mode.val(),
        "anwers": answer_array
    }
    question.val("");
    // time_mode.querySelector('[selected]').selected = true


   
 

document.querySelector("#btn_save").onclick = function(event){
    
    questions_array.push(question_json)
    //data to request
    data = {
        "name":quiz_name.val(),
        "questions":questions_array,
    }
    // ajax("https://268b-178-205-186-218.ngrok.io/api/user", "POST", response_reg, JSON.stringify(data));
    // function response_reg(dataArr){
       
    // }
    console.log(questions_array)
    console.log(data)

}



}
