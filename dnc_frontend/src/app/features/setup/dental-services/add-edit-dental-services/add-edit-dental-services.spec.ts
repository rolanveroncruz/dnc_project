import { ComponentFixture, TestBed } from '@angular/core/testing';

import { AddEditDentalServices } from './add-edit-dental-services';

describe('AddEditDentalServices', () => {
  let component: AddEditDentalServices;
  let fixture: ComponentFixture<AddEditDentalServices>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [AddEditDentalServices]
    })
    .compileComponents();

    fixture = TestBed.createComponent(AddEditDentalServices);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
