
    
function all(){
    let code = localStorage.getItem("code")
    answers_arr = []
    // data = {
    //     "type":  "InProgress",
    //     "current_question": {
    //         "answers": ["хуй", "халупа", "манда"],
    //         "question_type": "Poll",
    //         "quiz_id": 0,
    //         "text": "What is your favourite encoding format?",
    //         "time": 60
    //     },
    //     "current_question_id": 0,
    //     "time_left": 30
    //   }
   ajax(`https://b1f9-188-130-155-167.ngrok.io/games/${code}`, "GET", game_procces)
    // data = JSON.stringify(data)
    // game_procces(data);

    
    
    function game_procces(dataArray){
        window.onload = (() => {
            animation(Number(JSON.parse(dataArray).time_left));
                })
           
           createAnswer_(dataArray)
            let changes = dataArray.current_question_id-100;
            let timerId = setInterval(() => ajax(`https://b1f9-188-130-155-167.ngrok.io/games/${code}`, "GET", response_reg), 500)
            response_reg(data)
            function response_reg(dataArray){
                let changes2 = dataArray.current_question_id;
                if (changes != changes2){
                    if (JSON.parse(dataArray).type == "InProgress"){
                        console.log(11)
                        // data_ = JSON.parse(dataArray)
                       
                        if (JSON.parse(dataArray).current_question.question_type == "Poll" ||JSON.parse(dataArray).current_question.question_type == "Quiz" ){
                           
                                $('.Answer_button').click(function() {
                                  const el = $(this);
                                  console.log(el)
                                  console.log(el.text());
                                  answers_arr.push(el.text());
                                  ($('.Answer_button')).prop("disabled", true)
                                  el.css("opacity", "1")
                                });
            
            
                             if (JSON.parse(dataArray).current_question.time - JSON.parse(dataArray).time_left == 0){
                                data_json = {
                                    "player_name": localStorage.getItem("name"),
                                    "question_id": dataArray.current_question_id,
                                    "answers": answers_arr, 
            
                                }
                                ajax(`/games/${code}`, "PUT", response_reg, data_json);
                                function response_reg(data){
                                    all();
                             }
                           
                            }
                        else if (JSON.parse(dataArray).current_question.question_type == "Multquiz"){
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
                            if (JSON.parse(dataArray).current_question.time - JSON.parse(dataArray).time_left == 0){
                                data_json = {
                                    "player_name": localStorage.getItem("name"),
                                    "question_id": JSON.parse(dataArray).current_question_id,
                                    "answers": answers_arr, 
            
                                }
                                ajax(`https://b1f9-188-130-155-167.ngrok.io/games/${code}`, "PUT", response_reg, data_json)
                                function response_reg(data){
                                    all();
                                }
                             }
                            // console.log(answers_arr)
                        }
                     
                               
                                
                        
                            // console.log(document.getElementsByClassName("Answer_button"))
        
                        }
                        else if (JSON.parse(dataArray).current_question.question_type == "Finished"){
            
                            
                        }
    
                }
                changes = changes2;
        
                }
        
            }
        }
    }
       
    all()

