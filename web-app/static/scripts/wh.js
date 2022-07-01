// let timerId = setInterval(() => wait_hall(), 500);
function wait_hall(){
    // ajax("", "GET", response_reg)
    // addParticipant(2)
    // console.log(document.getElementsByClassName("person")[0].innerHTML)
    function response_reg(dataArr){
        let data = JSON.parse(dataArr);
        addParticipant(dataArr.length)
        for (let i = 0; i < dataArr.length; i+=1){
            document.getElementsByClassName("person")[i].innerHTML = dataArr[i]
        }
        $("#amount").val() = dataArr.length
        $("quiz_code_number").val()=localStorage.getItem("code")
    
    } 
}
