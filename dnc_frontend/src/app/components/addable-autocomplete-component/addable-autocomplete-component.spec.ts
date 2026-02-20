import { ComponentFixture, TestBed } from '@angular/core/testing';

import { AddableAutocompleteComponent } from './addable-autocomplete-component';

describe('AddableAutocompleteComponent', () => {
  let component: AddableAutocompleteComponent;
  let fixture: ComponentFixture<AddableAutocompleteComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [AddableAutocompleteComponent]
    })
    .compileComponents();

    fixture = TestBed.createComponent(AddableAutocompleteComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
