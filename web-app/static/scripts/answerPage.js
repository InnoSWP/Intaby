

function createAnswer_(dataArr) {
    var wrapper = document.getElementById("Button_wrapper")
    console.log( ($(".Question_wrapper"))[0])
    console.log(JSON.parse(dataArr))
    $(".Question_wrapper")[0].innerHTML = (JSON.parse(dataArr).current_question.text); 
    // number of questions
    for (let index = 1; index < Number(JSON.parse(dataArr).current_question.answers.length) + 1; index++) {
        const element = createAnswer(index, dataArr)
        wrapper.append(element)
    }
}

function createAnswer(_index, dataArr_) {
    dataArr_ = JSON.parse(dataArr_)
    var element = document.createElement("button")
    element.classList.add("Answer_button", "col-4", "btn", "py-4", "px-2", "mb-2", "text-white",
        "col-md-12", "col-lg-5", "position-relative")
    if (_index % 2 === 1) {
        element.classList.add("me-lg-2")
    }
    element.textContent = dataArr_.current_question.answers[_index-1]
    switch (_index) {
        case 1:
            element.style.backgroundColor = "#F83962"
            break

        case 2:
            element.style.backgroundColor = "#2EBFD5"
            break

        case 3:
            element.style.backgroundColor = "#FF943B"
            break

        case 4:
            element.style.backgroundColor = "#467CD3"
            break

        case 5:
            element.style.backgroundColor = "#8AC600"
            break

        case 6:
            element.style.backgroundColor = "#9D3FE1"
            break
    }
    // type of question
    if (dataArr_.current_question.question_type != "Poll") {
        addEvent(element, dataArr_.current_question.question_type)
        var badge = document.createElement('button')
        // console.log(sessionStorage.getItem("type"))
        switch (dataArr_.current_question.question_type) {
            case "Quiz":
                badge.classList.add('Badge', 'position-absolute', 'top-0', 'end-0', 'translate-middle-y', 'p-2', 'mt-3', 'me-2', 'border', 'border-light', 'rounded-circle', 'border-5')
                break
            case "Multiquiz":
                badge.classList.add('Badge', 'position-absolute', 'top-0', 'end-0', 'translate-middle-y', 'p-2', 'mt-3', 'me-2', 'border', 'border-light', 'rounded-2', 'border-5')
                break
        }
        badge.style.color = 'white'
        element.append(badge)
    }
    return element
}

var ifSelected = false
var selectedAnswer

function addEvent(_element, type) {
    // type of question
    switch (type) {
        case "Quiz":
            _element.addEventListener('click', () => {
                if (ifSelected === true) {
                    selectedAnswer.lastChild.classList.toggle('bg-dark')
                    _element.lastChild.classList.toggle('bg-dark')
                }
                else {
                    _element.lastChild.classList.toggle('bg-dark')
                }
                selectedAnswer = _element
                ifSelected = true
            })
            break
        case "Multiquiz":
            _element.addEventListener('click', () => {
                _element.lastChild.classList.toggle('bg-dark')
            })
            break
    }
}

// animation

 function animation(duration){
    var scale = document.getElementsByClassName('Time_scale')[0]
    var newStyles = document.createElement('style')
    newStyles.innerHTML = ".Time_scale {" +
        "transition: width " + Number(duration) + "s " + "ease-in;" +
    "width :" + 0
    "}"
    scale.append(newStyles)
}
