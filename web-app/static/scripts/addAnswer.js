var addButton = document.getElementById("add_button")
var clicks = 2
var wrapper = document.getElementsByClassName("Answer_add_wrapper")

addButton.addEventListener('click', function addAnswer() {
    console.log(1)
    clicks += 1
    if (clicks <= 6) {
        var newAnswer = document.createElement('textarea')
        newAnswer.classList.add("Answer_type", "col-12", "text-white", "px-2", "py-2", "mb-2", "mb-lg-4", "rounded")
        newAnswer.placeholder = "Answer " + clicks
        newAnswer.style.backgroundColor = "black"
        if (clicks === 3) {
            newAnswer.style.backgroundColor = "#FF943B";
        }
        if (clicks === 4) {
            newAnswer.style.backgroundColor = "#467CD3";
        }
        if (clicks === 5) {
            newAnswer.style.backgroundColor = "#8AC600";
        }
        if (clicks === 6) {
            newAnswer.style.backgroundColor = "#9D3FE1";
        }
        if (clicks < 6) {
            wrapper[0].append(newAnswer)
            wrapper[0].append(addButton)
        }
        else {
            wrapper[0].append(newAnswer)
            addButton.parentNode.removeChild(addButton)
        }
    }
})

var deleteButton = document.getElementsByClassName("delete_answer")

deleteButton[0].addEventListener('click', function DeleteAnswer() {
    if (clicks === 6) {
        wrapper[0].removeChild(wrapper[0].lastElementChild)
        wrapper[0].append(addButton)
        clicks -= 1
    }
    else if (clicks > 2) {
        var storage = wrapper[0].lastElementChild
        console.log(addButton)
        addButton.parentNode.removeChild(addButton)
        wrapper[0].removeChild(wrapper[0].lastElementChild)
        wrapper[0].append(storage)
        clicks -= 1
    }

})