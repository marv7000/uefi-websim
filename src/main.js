// Don't reload the page on submit.
var form = document.getElementById("submitter");
function handleForm(event) { event.preventDefault(); }
form.addEventListener('submit', handleForm);

// Variable Import
async function readText(event) {
	const file = event.target[0].files.item(0);
	const text = await file.text();

	return text;
}
function import_vars(event) {
	readText(event).then(function (text) {
		// Split all lines
		var lines = text.split(/\r?\n/).filter(n => n);
		// Split each line by ": "
		for (var i = 0; i < lines.length; i++) {
			var [id, val] = lines[i].split(": ");
			// Set elements with the respective id
			var elem = document.getElementById(id);
			if (elem && elem.type) {
				if (elem.type === 'checkbox') {
					elem.checked = (val === 'true');
				} else if (elem.type === 'select-one') {
					elem.value = val;
				}
			}
		}
	})
}

// Variable Export
function export_vars() {
	var result = "";
	var config = document.getElementsByClassName("config");

	for (i = 0; i < config.length; i++) {
		result += config[i].id + ": ";
		if (config[i].type) {
			if (config[i].type === 'checkbox') {
				result += config[i].checked;
			} else if (config[i].type === 'select-one') {
				result += config[i].value;
			}
		}
		result += '\n'
	}
	return result;
}
function download(filename, text) {
	var element = document.createElement('a');
	element.setAttribute('href', 'data:text/plain;charset=utf-8,' + encodeURIComponent(text));
	element.setAttribute('download', filename);

	element.style.display = 'none';
	document.body.appendChild(element);

	element.click();

	document.body.removeChild(element);
}

// Collapsibles
var coll = document.getElementsByClassName("collapsible");
var i;

for (i = 0; i < coll.length; i++) {
	coll[i].addEventListener("click", function () {
		this.classList.toggle("active");
		var content = this.nextElementSibling;
		if (content.style.display === "block") {
			content.style.display = "none";
		} else {
			content.style.display = "block";
		}
	});
}

// Update home page values
for (let key in custom_variables) {
	var elem = document.getElementById(key);
	if (elem) {
		elem.textContent = custom_variables[key];
	}
}

// Category tabs
function openCategory(evt, cat) {
	var i, tabcontent, tablinks;
	tabcontent = document.getElementsByClassName("category");
	for (i = 0; i < tabcontent.length; i++) {
		tabcontent[i].style.display = "none";
	}
	tablinks = document.getElementsByClassName("tablinks");
	for (i = 0; i < tablinks.length; i++) {
		tablinks[i].className = tablinks[i].className.replace(" active", "");
	}
	document.getElementById(cat).style.display = "block";
	evt.currentTarget.className += " active";
}
