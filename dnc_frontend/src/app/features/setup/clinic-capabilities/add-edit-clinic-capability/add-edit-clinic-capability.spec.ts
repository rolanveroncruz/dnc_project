import { ComponentFixture, TestBed } from '@angular/core/testing';

import { AddEditClinicCapability } from './add-edit-clinic-capability';

describe('AddEditClinicCapability', () => {
  let component: AddEditClinicCapability;
  let fixture: ComponentFixture<AddEditClinicCapability>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [AddEditClinicCapability]
    })
    .compileComponents();

    fixture = TestBed.createComponent(AddEditClinicCapability);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
