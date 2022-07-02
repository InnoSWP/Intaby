let timerId = setInterval(() => game_procces(), 500);
answers_arr = []
// data = {
//     "type":  "InProgress",
//     "current_question": {
//         "answers": [],
//         "question_type": "Poll",
//         "quiz_id": 0,
//         "text": "What is your favourite encoding format?",
//         "time": 60
//     },
//     "current_question_id": 0,
//     "time_left": 60
//   }
let current_question_id = data.current_question_id;

function game_procces(dataArray){
     ajax(`/games/${code}`, "GET", response_reg)
    
    function response_reg(dataArray){
        if (dataArray.type == "InProgress"){
            console.log(11)
            // data_ = JSON.parse(dataArray)
            createAnswer_(dataArray)
            if (dataArray.current_question.question_type == "Poll" ||dataArray.current_question.question_type == "Quiz" ){
               
                    $('.Answer_button').click(function() {
                      const el = $(this);
                      console.log(el)
                      console.log(el.text());
                      answers_arr.push(el.text());
                      ($('.Answer_button')).prop("disabled", true)
                      el.css("opacity", "1")
                    });


                 if (data.current_question.time - data.time_left == 0){
                    data_json = {
                        "player_name": localStorage.getItem("name"),
                        "question_id": current_question_id,
                        "answers": answers_arr, 

                    }
                    ajax(`/games/${code}`, "PUT", response_reg, data_json)
                 }
               
                }
            else if (dataArray.current_question.question_type == "Multquiz"){
                $('.Answer_button').click(function() {
                    const el = $(this);
                    console.log(el)
                    
                    console.log(answers_arr)

                    console.log(answers_arr.includes(el.text()))
                    if (answers_arr.includes(el.text())){
                        answers_arr.splice(answers_arr.indexOf(el.text()), 1)
                    }
                    answers_arr.push(el.text());
                })
                if (data.current_question.time - data.time_left == 0){
                    data_json = {
                        "player_name": localStorage.getItem("name"),
                        "question_id": current_question_id,
                        "answers": answers_arr, 

                    }
                    ajax(`/games/${code}`, "PUT", response_reg, data_json)
                 }
                // console.log(answers_arr)
            }
         
                   
                    
            
                // console.log(document.getElementsByClassName("Answer_button"))
            }
            else if (dataArray.current_question.question_type == "Finished"){

                
            }
            






        }

    }
