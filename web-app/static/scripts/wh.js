let timerId_ = setInterval(() => wait_hall(), 500);
let code = localStorage.getItem("code");
let count_players = 0;
function wait_hall(){
    ajax(`https://bfd5-188-130-155-167.ngrok.io/games/${code}`, "GET", response_reg)
    // addParticipant(2)
    // console.log(document.getElementsByClassName("person")[0].innerHTML)
    function response_reg(dataArr){
        let data = JSON.parse(dataArr);
        console.log()
       

        if (data.type == "Lobby"){
            addParticipant(data.players.length - count_players);
            for (let i = count_players; i < data.players.length; i+=1){
                document.getElementsByClassName("person")[i].innerHTML = data.players[i];
            }
             $("#amount")[0].innerHTML = (data.players.length);
             console.log(data.players.length)
             console.log( $("#amount"))
            document.getElementsByClassName("quiz_code_number")[0].innerHTML = code;
            console.log(document.getElementsByClassName("quiz_code_number")[0]);
            console.log(code)
        }
        else if (data.type == "InProgress"){
            window.location.href="../templates/page_of_answer.html"
        }
        count_players = data.players.length;
        console.log(count_players)
        
        
       
    
    
    } 
}
