const js = import("./pkg");
js.then((js) => {
  const field = document.getElementById("field");
    field.addEventListener("change", (e) => {
	console.log(e);
	const text = e.target.value;
	const result = js.validate_string(text);
	console.log(result);

	/*
	const contact = document.getElementById("contact");
	const res2 = js.newSRU(contact, ...);
	js.SRUtoString(res2);


	const sru_builder = new pkg.SRUBuilder();
	// ...
	() => {
	    let r = sru_builder.set_media_id(x);
	    if(r == true)
		return;
	    reportError(r);
	};
    // ...
    let sru = sru_builder.build();
    const res = sru.to_string();
	*/
	
  });
});
