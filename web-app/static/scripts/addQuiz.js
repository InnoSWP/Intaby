
window.onload = addQuiz(7)


function addQuiz(quizNumber) {
    var wrapper = document.getElementById('quiz_storage')
    for (let index = 0; index < quizNumber; index++) {
        var copy = createQuiz()
        wrapper.appendChild(copy)
    }
}

function createQuiz() {

    var title = document.createElement('div')
    title.classList.add('title')
    title.textContent ="TITLE"
    var secondary_text = document.createElement('div')
    secondary_text.classList.add('secondary_text')
    secondary_text.textContent="Secondary text"
    var header = document.createElement('div')
    header.classList.add('add_quiz_header', 'p-2', 'col-12')
    header.appendChild(title)
    header.appendChild(secondary_text)

    var quizText = document.createElement('div')
    quizText.classList.add('quiz_text', 'text-center', 'col-10')
    quizText.style.color = "#DBE4ED"
    quizText.textContent = "Quiz"

    var description = document.createElement('div')
    description.classList.add('description')
    description.textContent = "desription"
    var edit = document.createElement('div')
    edit.classList.add('edit')
    edit.textContent = "EDIT"
    var footer = document.createElement('div')
    footer.classList.add('add_quiz_footer', 'p-2', 'col-12')
    footer.appendChild(description)
    footer.appendChild(edit)

    var wrapper1 = document.createElement('div')
    wrapper1.classList.add('d-flex', 'flex-column', 'justify-content-between', 'align-items-center')
    wrapper1.style.height = "100%"
    wrapper1.appendChild(header)
    wrapper1.appendChild(quizText)
    wrapper1.appendChild(footer)

    var wrapper2 = document.createElement('div')
    wrapper2.classList.add('quiz_button', 'mx-4', 'my-4', 'col-8', 'col-md-3', 'position-relative')
    wrapper2.id = "quiz_button"
    wrapper2.style.height = "70%"
    wrapper2.appendChild(wrapper1)
    return wrapper2
}