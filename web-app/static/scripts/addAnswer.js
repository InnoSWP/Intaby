var addButton = document.getElementsByClassName("add_answer_quiz")
var clicks = 2

addButton[0].addEventListener('click', function addAnswer() {
    clicks += 1
    if (clicks <= 6) {
        var newAnswer = document.createElement('textarea')
        newAnswer.classList.add("Answer_type", "col-12", "text-white", "px-2", "py-2", "mb-2", "mb-lg-4", "mb-2", "mb-lg-4", "rounded")
        newAnswer.placeholder = "Answer " + clicks
        newAnswer.style.backgroundColor = "black"
        var wrapper = document.getElementsByClassName("Answer_add_wrapper")
        if (clicks === 3) {
            newAnswer.style.backgroundColor = "#FF943B";
        }
        if (clicks === 4) {
            newAnswer.style.backgroundColor = "#9D3FE1";
        }
        if (clicks === 5) {
            newAnswer.style.backgroundColor = "#68DC49";
        }
        if (clicks === 6) {
            newAnswer.style.backgroundColor = "#FFE255";
        }
        if (clicks < 6) {
            wrapper[0].append(newAnswer)
            wrapper[0].append(addButton[0])
        }
        else {
            wrapper[0].append(newAnswer)
            addButton[0].parentNode.removeChild(addButton[0])
        }
    }
})
