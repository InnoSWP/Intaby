console.log(JSON.parse(localStorage.getItem("user_data")))
let user_token = JSON.parse(JSON.parse(localStorage.getItem("user_data")))
ajax("", POST, response_reg, user_token)

function response_reg(data){
    console.log(data)
    if (this.readyState == 4 && this.status==200){
        $("#username").val();
        $("#")
    }
}