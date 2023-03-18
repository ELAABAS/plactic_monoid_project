all:		pdf

tex:
	@latex main.tex

a4:
	@latex main.tex
	@bibtex main
	@latex main.tex
	@latex main.tex

pdf:
	@pdflatex main.tex
	@bibtex main
	@pdflatex main.tex
	@pdflatex main.tex

svn:
	@svn ci -m ""

clean:
	@echo "Cleaning files..."
	@rm -f *.aux
	@rm -f *.bbl
	@rm -f *.blg
	@rm -f *.dvi
	@rm -f *.log
	@rm -f *.out
	@rm -f *.ps
	@rm -f *.toc
	@rm -f *~

