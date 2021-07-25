from django.shortcuts import render

import logging


logger = logging.getLogger(__name__)

def add(request):
    if request.method == 'POST':
        print("New file has been dropped:", request.FILES)
        # logger.info('message')
    
    return render(request, 'files/add.html', {})
    