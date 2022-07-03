
   
let flag = true
let changes = -100;
// function all(){

    console.log("123")
    
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
//    ajax(`https://23aa-188-130-155-167.ngrok.io/games/${code}`, "GET", game_procces)
    // data = JSON.stringify(data)
    // game_procces(data);
    // game_procces();
   
        
    
    // function game_procces(){
    let timerId = setInterval(() => ajax(`https://bfd5-188-130-155-167.ngrok.io/games/${code}`, "GET", response_reg), 500);
    // ($('.Answer_button')).prop("disabled", false)
    
    // console.log(changes)

    function response_reg(dataArray){
        let changes2 = JSON.parse(dataArray).current_question_id;
        // console.log(changes2)
        //  console.log(JSON.parse(dataArray))
        // console.log(localStorage.getItem("name"))
        if (changes != changes2 && JSON.parse(dataArray).type != "Finished"){

            animation(Number(JSON.parse(dataArray).current_question.time));
            $('.Answer_button').remove();
            createAnswer_(dataArray);
            // console.log(changes)
            if (changes2 != 0){
                // console.log("пиздец")
                data_json = {
                    "player_name": localStorage.getItem("name"),
                    "question_id": changes2,
                    "answers": answers_arr, 

                }
                
                ajax(`https://bfd5-188-130-155-167.ngrok.io/games/${code}`, "PUT", response_reg, JSON.stringify(data_json));
                function response_reg(data){
                    answers_arr = []
                    console.log(data_json)
                    // $('.Answer_button').remove()
                    // all();
                }
            }


            // else{
                console.log("Зашли в прогресс")
                // data_ = JSON.parse(dataArray)
                if (JSON.parse(dataArray).current_question.question_type == "Poll" || JSON.parse(dataArray).current_question.question_type == "Quiz" ){
                        // console.log("зашли в полл")
                        // console.log($('.Answer_button'))
                        // console.log(JSON.parse(dataArray))
                        $('.Answer_button').click(function() {
                            const el = $(this);
                        //   console.log(el)
                        //   console.log(el.text());
                            answers_arr.push(el.text());
                            ($('.Answer_button')).prop("disabled", true)
                            el.css("opacity", "1")
                        });
                }
                else{
                    $('.Answer_button').click(function() {
                        const el = $(this);
                    
                        // if (answers_arr.includes(el.text())){
                        //     answers_arr.splice(answers_arr.indexOf(el.text()), 1)
                        // }
                        answers_arr.push(el.text());
                    })
                }
                        // console.log(document.getElementsByClassName("Answer_button"))
            // }
        }
        else if (JSON.parse(dataArray).type == "Finished"){

              window.location.href = "../templates/leaderboard.html";
            flag = false   
        }
        changes = changes2;
 // }
    }
    
// if (flag){
//     $('.Answer_button').remove()
//     all();   
// }
  

      
  

