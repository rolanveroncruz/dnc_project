import { Component,Input } from '@angular/core';
import {FormGroup, ReactiveFormsModule} from '@angular/forms';
import {MatFormFieldModule} from '@angular/material/form-field';
import {MatInputModule} from '@angular/material/input';
import {DentalService} from '../basic-services-tab-component/basic-services-tab-component';

@Component({
  selector: 'app-special-services-tab-component',
  imports: [
    ReactiveFormsModule,
    MatFormFieldModule,
    MatInputModule
  ],
  templateUrl: './special-services-tab-component.html',
  styleUrl: './special-services-tab-component.scss',
  standalone: true
})
export class SpecialServicesTabComponent {
  @Input({required: true}) services: DentalService[] =[];
  @Input({required: true}) ratesGroup!: FormGroup;
  @Input() disabled = false;

  controlName(serviceId:number){
    return String(serviceId);
  }


}
