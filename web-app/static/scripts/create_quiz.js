
let questions_array=[]
let answer_array=[]

document.querySelector("#btn_create").onclick = function(event){
    event.preventDefault();

    let quiz_name = $("#quiz_name");  
    let question = $("#question");

    question.val("");
 
    $(".Answer_type").each(function(){
        console.log(this.value)
        //push answers to array
        answer_array.push(this.value)
        this.value =""
    })
   
 

document.querySelector("#btn_save").onclick = function(event){
    question_json = {
        "anwers": answer_array
    }
    questions_array.push(question_json)
    //data to request
    data = {
        "name":quiz_name.val(),
        "questions":questions_array,
    }
    ajax("https://268b-178-205-186-218.ngrok.io/api/user", "POST", response_reg, JSON.stringify(data));
    function response_reg(dataArr){
       
    }
    console.log(questions_array)
    console.log(data)

}



}
