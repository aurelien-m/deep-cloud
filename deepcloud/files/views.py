from django.shortcuts import render


def add(request):
    if request.method == 'POST':
        pass
    
    return render(request, 'files/add.html', {})
    