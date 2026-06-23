import { ComponentFixture, TestBed } from '@angular/core/testing';

import { FindDentistComponent } from './find-dentist-component';

describe('FindDentistComponent', () => {
  let component: FindDentistComponent;
  let fixture: ComponentFixture<FindDentistComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [FindDentistComponent]
    })
    .compileComponents();

    fixture = TestBed.createComponent(FindDentistComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
